import { Alert, Pane } from "evergreen-ui";
import React from "react";
import { DocumentPageProps } from "../document/document";

export const LeaseFurnishedTemplatePart2: React.FunctionComponent<
  DocumentPageProps
> = ({
  form,
  getItem,
}) => {
  return (
    <Pane>
      <h2 id="iii-date-de-prise-d-effet-et-duree-et-du-contrat">
        III. DATE DE PRISE D'EFFET ET DUREE ET DU CONTRAT
      </h2>
      <p>
        La durée du contrat et sa date de prise d'effet sont ainsi définies :
      </p>
      <p>
        <strong>
          A. Date de prise d'effet du contrat :
        </strong>
      </p>
      {getItem("contract_effect_date")}
      <p>
        <strong>B. Durée du contrat</strong> :
      </p>
      {getItem("contract_duration")}
      <Alert
        intent="none"
        marginBottom={32}
      >
        A l'exception des locations consenties à un étudiant pour une durée de
        neuf mois, les contrats de location de logements meublés sont reconduits
        tacitement à leur terme pour une durée d'un an et dans les mêmes
        conditions. Le locataire peut mettre fin au bail à tout moment, après
        avoir donné congé. Le bailleur peut, quant à lui, mettre fin au bail à
        son échéance et après avoir donné congé, soit pour reprendre le logement
        en vue de l'occuper lui-même ou une personne de sa famille, soit pour le
        vendre, soit pour un motif sérieux et légitime. Les contrats de
        locations meublées consenties à un étudiant pour une durée de neuf mois
        ne sont pas reconduits tacitement à leur terme et le locataire peut
        mettre fin au bail à tout moment, après avoir donné congé. Le bailleur
        peut, quant à lui, mettre fin au bail à son échéance et après avoir
        donné congé.
      </Alert>
      <h2 id="iv-conditions-financieres">IV. CONDITIONS FINANCIERES</h2>
      <p>Les parties conviennent des conditions financières suivantes :</p>
      <p>
        <strong>A. Loyer</strong>
      </p>
      <p>
        <strong>1° Fixation du loyer initial :</strong>
        a) Montant du loyer mensuel :
      </p>
      {getItem("contract_rent_amount")}
      <p>
        (29). b) Le cas échant, Modalités particulières de fixation initiale du
        loyer applicables dans certaines zones tendues (30) :
      </p>
      <ul>
        <li>
          le loyer du logement objet du présent contrat est soumis au décret
          fixant annuellement le montant maximum d'évolution des loyers à la
          relocation :
          {getItem("contract_max_evolution_relocation")}
        </li>
        <li>
          le loyer du logement objet du présent contrat est soumis au loyer de
          référence majoré fixé par arrêté préfectoral :
          {getItem("contract_rent_majoration_decree")}
        </li>
        <span>
          <li>
            montant du loyer de référence :
            {getItem("contract_rent_maj_decree_reference_amount")}{" "}
            / Montant du loyer de référence majoré :
            {getItem("contract_rent_maj_decree_increased_amount")}
          </li>
          <li>
            Le cas échéant, Complément de loyer :
            {getItem("contract_rent_complement")}
          </li>
        </span>
      </ul>
      <p>
        c) Le cas échéant, Informations relatives au loyer du dernier locataire
        : (31)
      </p>
      <div>
        {getItem("contract_tenant_last_rent_amount")}
      </div>
      <p>
        <strong>2° Le cas échéant, Modalités de révision :</strong>
      </p>
      {getItem("contract_rent_irl")}
      <Alert
        intent="none"
        title="A quoi sert l'IRL ?"
        marginBottom={32}
      >
        L'indice de référence des loyers (IRL) sert de base pour réviser les
        loyers des logements vides ou meublés. Il fixe les plafonds des
        augmentations annuelles des loyers que peuvent exiger les propriétaires.
      </Alert>
      <p>
        <strong>B. Charges récupérables</strong>
      </p>
      <ol>
        <li>
          Modalité de règlement des charges récupérables :
          {getItem("contract_charges_payment_method")}
        </li>
        <li>
          Le cas échéant, Montant des provisions sur charges ou du forfait de
          charges :
          {getItem("contract_charges_amount")}
        </li>
        <li>
          Le cas échéant, Modalités de révision du forfait de charges (32):
          {getItem("contract_charges_revision_method")}
        </li>
      </ol>
      <div style={{ pageBreakAfter: "always" }} />
      <p>
        <strong>C. Le cas échéant, En cas de colocation</strong>
        , souscription par le bailleur d'une assurance pour le compte des
        colocataires (33) :
      </p>
      {getItem("contract_colocation_insurance_lender")}
      <p>
        a) Montant total annuel récupérable au titre de l'assurance pour compte
        des colocataires (34) :
      </p>
      {getItem("contract_colocation_insurance_total_amount")}
      <p>
        b) Montant récupérable par douzième :
      </p>
      {getItem("contract_colocation_insurance_monthly_amount")}
      <p>
        <strong>D. Modalités de paiement</strong>
      </p>
      <ul>
        <li>
          périodicité du paiement (35) :
          {getItem("contract_rent_periodicity")}
        </li>
        <li>
          paiement :
          {getItem("contract_rent_payment_type")}
        </li>
        <li>
          date ou période de paiement :
          {getItem("contract_rent_payment_date")}
        </li>
        <li>
          le cas échéant, Lieu de paiement :
          {getItem("contract_rent_payment_place")}
        </li>
        <li>
          le cas échéant, Montant total du à la première échéance de paiement
          pour une période complète de location :
          {getItem("contract_rent_first_amount")}
        </li>
      </ul>
      <p>
        <strong>
          E. Le cas échéant, exclusivement lors d'un renouvellement de contrat,
          Modalités de réévaluation d'un loyer manifestement sous-évalué
        </strong>
      </p>
      <ol>
        <li>
          Montant de la hausse ou de la baisse de loyer mensuelle :
          {getItem("contract_rent_underestimated_monthly_variation")}
        </li>
        <li>
          Modalité d'application annuelle de la hausse :
          {getItem("contract_rent_underestimated_method")}
        </li>
      </ol>
    </Pane>
  );
};
